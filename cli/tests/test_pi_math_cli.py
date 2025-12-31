import pytest
from click.testing import CliRunner
from cli.pi_math_cli import pi_based_hash

def test_pi_based_hash():
    runner = CliRunner()
    result = runner.invoke(pi_based_hash, ['--data', 'test'])
    assert len(result.output.split(': ')[1].strip()) == 128  # SHA3-512 hex
