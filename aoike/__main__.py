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


@cli.command(name='build')
@click.option("--src-dir", default="./", help="Source Dir", type=str)
def build_command(src_dir):
    from aoike.commands.build import build
    build(src_dir = src_dir)


if __name__ == '__main__':
    cli()
