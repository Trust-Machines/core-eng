import logging

import click

import commands

logger = logging.getLogger()
logger.setLevel(logging.DEBUG)


@click.group()
@click.option('--debug/--no-debug', default=False)
def entry_point(debug):
    version = '0.0.1'
    print(f'btctool {version}')


def main():
    entry_point.add_command(commands.dl)
    entry_point.add_command(commands.ls)
    entry_point.add_command(commands.tx)
    entry_point.add_command(commands.sg)
    entry_point()


if __name__ == "__main__":
    main()
