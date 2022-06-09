import asyncio
import logging
from functools import partial
from typing import Any, Awaitable, Callable, Coroutine, Optional

from aiohttp.web import Application, GracefulExit

logger = logging.getLogger(__name__)


WorkerType = Callable[[], Awaitable]
ExecutorType = Callable[[Any], Coroutine]


def bail_out() -> None:
    raise GracefulExit


class Worker:
    def __init__(self, worker_func: WorkerType, logger: logging.Logger = logger):
        self.worker_func = worker_func
        self.logger = logger

    async def __call__(self) -> None:
        try:
            await self.worker_func()
        except Exception:
            self.logger.exception(
                "unhandled exception - bailing out",
            )
            asyncio.get_event_loop().call_soon(bail_out)
        else:
            self.logger.exception("worker bailing out")
            asyncio.get_event_loop().call_soon(bail_out)


class Workers:
    """Maintain a pool of asynchronous workers

    They maintain in memory normalized data to stream to other consumers
    """

    def __init__(self) -> None:
        super().__init__()
        self.logger: logging.Logger = logger.getChild(self.name)
        self._output: asyncio.Queue = asyncio.Queue()
        self._workers: list[Worker] = []
        self._tasks: Optional[tuple[asyncio.Task, ...]] = None
        self.add_worker(Worker(self._async_execution))

    @property
    def name(self) -> str:
        """This is my name"""
        return self.__class__.__name__.lower()

    @classmethod
    def setup(cls, app: Application, **kwargs: Any) -> "Workers":
        """Create the workers and register startup and shutdown events with the app"""
        worker = cls(**kwargs)
        app.on_startup.append(worker.on_startup)
        app.on_shutdown.append(worker.on_shutdown)
        return worker

    def status(self) -> dict:
        """Return a status dict or raise an exception if in a bad state"""
        return {}

    def execute(self, executor: ExecutorType, *args: Any) -> None:
        self._output.put_nowait(partial(executor, *args))

    def add_worker(self, worker: Worker) -> None:
        self._workers.append(worker)

    async def on_startup(self, app: Application) -> None:
        """register startup event with main app"""
        self._tasks = tuple(asyncio.create_task(worker()) for worker in self._workers)

    async def on_shutdown(self, app: Application) -> None:
        """register shutdown event with main app"""
        if self._tasks:
            self.logger.warning("closing %d background tasks", len(self._tasks))
            for task in self._tasks:
                task.cancel()
            try:
                await asyncio.gather(*self._tasks)
            except asyncio.CancelledError:
                pass

    async def _async_execution(self) -> None:
        """Coroutine for executing async commands"""
        while True:
            executor = await self._output.get()
            await executor()
            await asyncio.sleep(0)
