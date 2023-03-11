FROM ubuntu:20.04

# Keeps Python from generating .pyc files in the container
ENV PYTHONDONTWRITEBYTECODE 1
# Turns off buffering for easier container logging
ENV PYTHONUNBUFFERED 1

# Install and setup poetry
RUN apt-get update \
    && apt-get install -y python3 python3-pip python3-venv libgmp3-dev libusb-1.0-0-dev \
    && pip3 install poetry

# Install the btctool app
WORKDIR /app
COPY . .
RUN poetry install --no-interaction --no-ansi

ENTRYPOINT ["poetry", "run", "python", "btctool/btctool.py"]

