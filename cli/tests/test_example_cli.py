import pytest
from click.testing import CliRunner
from cli.examples_cli import merchant_example

def test_merchant_example():
    runner = CliRunner()
    result = runner.invoke(merchant_example, ['--product', 'test', '--base-price', '1.0'])
    assert "priced at" in result.output
