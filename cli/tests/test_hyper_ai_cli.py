import pytest
from click.testing import CliRunner
from cli.hyper_ai_cli import predict

def test_predict():
    runner = CliRunner()
    result = runner.invoke(predict, ['--input', '50'])
    assert "Ensemble Prediction" in result.output
