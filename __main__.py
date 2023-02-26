import click


@click.group()
def cli():
    pass


@cli.command(name='init')
def init_command():
    pass


@cli.command(name='serve')
def serve_command():
    from aoike.commands import serve
    serve.serve()


if __name__ == '__main__':
    cli()
