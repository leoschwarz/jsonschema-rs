import os
import json
import jsonschema_rs


SUPPORTED_DRAFTS = ("draft7", "draft6", "draft4")


def load_file(path):
    with open(path) as fd:
        for block in json.load(fd):
            yield block


def pytest_generate_tests(metafunc):
    cases = [
        (block["schema"], test["data"], test["valid"], test["description"])
        for draft in SUPPORTED_DRAFTS
        for root, dirs, files in os.walk(f"../tests/suite/tests/{draft}/")
        for filename in files
        for block in load_file(os.path.join(root, filename))
        for test in block["tests"]
    ]
    metafunc.parametrize("schema, instance, expected, description", cases)


def test_draft(schema, instance, expected, description):
    assert jsonschema_rs.is_valid(schema, instance) is expected, f"{description}: {schema} | {instance}"
