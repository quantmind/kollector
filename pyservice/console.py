import asyncio

from rich.align import Align
from rich.console import Console, Group
from rich.live import Live
from rich.table import Table

from .book import ZERO, Book
from .gateway import Publisher
from .workers import Worker, Workers


class ConsoleUI(Workers, Publisher):
    def __init__(self, levels: int = 10) -> None:
        super().__init__()
        self.levels = levels
        self.console = Console()
        self._books: asyncio.Queue[list[Align]] = asyncio.Queue()
        self.add_worker(Worker(self._live))

    async def publish_books(self, books: dict[str, Book]) -> None:
        """Publish data to console."""
        if books:
            tables = [self.create_table(pair, books[pair]) for pair in sorted(books)]
            await self._books.put(tables)

    def create_table(self, symbol: str, book: Book) -> Align:
        """Create a table for a given orderbook

        Display the top self.levels only
        """
        w = 15
        table = Table(
            title=(
                f"[b]{symbol.upper()}[/b] mid: "
                f"{book.mid()} imbalance: {round(book.imbalance(), 2)}"
            )
        )
        table.add_column("Bid CumVol", style="bright_green", justify="right", width=w)
        table.add_column("Bid Volume", style="bright_green", justify="right", width=w)
        table.add_column("Bid Price", style="bright_green", justify="right", width=w)
        table.add_column("Ask Price", style="bright_red", justify="left", width=w)
        table.add_column("Ask Volume", style="bright_red", justify="left", width=w)
        table.add_column("Ask CumVol", style="bright_red", justify="left", width=w)
        ask_cum = ZERO
        bid_cum = ZERO
        for (bid_price, bid_volume), (ask_price, ask_volume), _ in zip(
            book.bids.items(), book.asks.items(), range(self.levels)
        ):
            ask_cum += ask_volume
            bid_cum += bid_volume
            table.add_row(
                str(bid_cum),
                str(bid_volume),
                str(bid_price),
                str(ask_price),
                str(ask_volume),
                str(ask_cum),
            )
        return Align.center(table)

    async def _live(self) -> None:
        """Coroutine for refreshing the console"""
        with Live(console=self.console, screen=True, auto_refresh=False) as live:
            while True:
                books = await self._books.get()
                live.update(Group(*books), refresh=True)
