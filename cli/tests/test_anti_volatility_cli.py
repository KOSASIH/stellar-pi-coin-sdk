import pytest
from click.testing import CliRunner
from cli.anti_volatility_cli import check_volatility

def test_check_volatility():
    runner = CliRunner()
    result = runner.invoke(check_volatility, ['--asset', 'bitcoin'])
    assert "is_rejected" in result.output
    assert json.loads(result.output)["is_rejected"] == True
