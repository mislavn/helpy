from rust_server._core import RustClass
import pytest


def test_calculator_create():
    calculator = RustClass()
    assert calculator.counter(0, 0) == 0


def test_calculator_sum_array():
    calculator = RustClass()
    input = {"array1": [2, 3], "array2": [4, 5]}
    response = calculator.sum_array(input, 0, 0)
    assert response["operation_counter"] == 1
    assert response["array"] == [6, 8]


def test_invalid_calculator_sum_array():
    calculator = RustClass()
    input = {"array1": [2], "array2": [4, 5]}
    try:
        calculator.sum_array(input, 0, 0)
        pytest.fail("The method call should have failed")
    except ValueError:
        pass
