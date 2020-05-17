from contextlib import suppress

import pytest
from hypothesis import given, strategies as st
from jsonschema_rs import is_valid


json = st.recursive(
    st.none() | st.booleans() | st.floats() | st.integers() | st.text(),
    lambda children: st.lists(children, min_size=1) | st.dictionaries(st.text(), children, min_size=1),
)


@given(instance=json)
def test_instance_processing(instance):
    with suppress(Exception):
        is_valid(True, instance)


@given(instance=json)
def test_schema_processing(instance):
    with suppress(Exception):
        is_valid(instance, True)


def test_invalid_schema():
    with pytest.raises(ValueError):
        is_valid(2 ** 64, True)


def test_invalid_type():
    with pytest.raises(ValueError):
        is_valid(set(), True)


@given(minimum=(st.integers() | st.floats(allow_nan=False, allow_infinity=False)).map(abs))
def test_minimum(minimum):
    with suppress(SystemError):
        assert is_valid({"minimum": minimum}, minimum)
        assert is_valid({"minimum": minimum}, minimum - 1) is False


@given(maximum=(st.integers() | st.floats(allow_nan=False, allow_infinity=False)).map(abs))
def test_maximum(maximum):
    with suppress(SystemError):
        assert is_valid({"maximum": maximum}, maximum)
        assert is_valid({"maximum": maximum}, maximum + 1) is False
