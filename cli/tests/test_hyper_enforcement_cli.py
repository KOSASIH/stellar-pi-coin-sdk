import pytest
from click.testing import CliRunner
from cli.hyper_enforcement_cli import check_compliance

def test_check_compliance():
    runner = CliRunner()
    result = runner.invoke(check_compliance, ['--entity', 'merchant_pi'])
    assert "compliant" in result.output
