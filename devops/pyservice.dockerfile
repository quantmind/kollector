FROM python:3.10

ENV PYTHONPATH $PYTHONPATH:/app

WORKDIR /app
#
# INSTALL python dependencies & test
COPY ./pyproject.toml ./poetry.lock ./
RUN pip install -U poetry
RUN poetry config virtualenvs.create false
RUN poetry install --no-interaction --no-ansi
RUN rm pyproject.toml poetry.lock
RUN rm -rf /root/.cache

COPY . .

RUN pytest
RUN ./devops/lint-py
