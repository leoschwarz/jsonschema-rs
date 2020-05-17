import json
import timeit
from textwrap import dedent
import fastjsonschema
import jsonschema


def load_json(filename):
    with open(filename) as fd:
        return json.load(fd)


BIG_SCHEMA = load_json("../benches/canada_schema.json")
BIG_INSTANCE = load_json("../benches/canada.json")
SMALL_SCHEMA = load_json("../benches/small_schema.json")
SMALL_INSTANCE_VALID = [9, 'hello', [1, 'a', True], {'a': 'a', 'b': 'b', 'd': 'd'}, 42, 3]

# Compiled fastjsonschema validators
fast_validate_big = fastjsonschema.compile(BIG_SCHEMA)
fast_validate_small = fastjsonschema.compile(SMALL_SCHEMA)
jsonschema_validate_big = jsonschema.validators.validator_for(BIG_SCHEMA)(BIG_SCHEMA)
jsonschema_validate_small = jsonschema.validators.validator_for(SMALL_SCHEMA)(SMALL_SCHEMA)


ITERATIONS_BIG = 10
ITERATIONS_SMALL = 10000
setup = dedent("""
    import json
    import jsonschema_rs
    import jsonschema
    import fastjsonschema
    from __main__ import (
        BIG_SCHEMA, 
        BIG_INSTANCE, 
        SMALL_SCHEMA,
        SMALL_INSTANCE_VALID,
        fast_validate_small,
        fast_validate_big,
        jsonschema_validate_big,
        jsonschema_validate_small
    )
""")


def run(code, name, number):
    result = timeit.timeit(code, setup, number=number)
    print(f"{name} => {result:.5f}")


def bench_object(schema, instance, number, type_):
    code = f"jsonschema_rs.is_valid({schema}, {instance})"
    run(code, f"{type_}: rust not compiled", number)


def bench_jsonschema_compiled(func, instance, number, type_):
    code = f"{func}.is_valid({instance})"
    run(code, f"{type_}: jsonschema compiled", number)


def bench_jsonschema_not_compiled(schema, instance, number, type_):
    code = f"jsonschema.validate({instance}, {schema})"
    run(code, f"{type_}: jsonschema not compiled", number)


def bench_fastjsonschema_compiled(func, instance, number, type_):
    code = f"{func}({instance})"
    run(code, f"{type_}: fastjsonschema compiled", number)


def bench_fastjsonschema_not_compiled(schema, instance, number, type_):
    code = f"fastjsonschema.compile({schema})({instance})"
    run(code, f"{type_}: fastjsonschema not compiled", number)


if __name__ == '__main__':
    bench_object("SMALL_SCHEMA", "SMALL_INSTANCE_VALID", ITERATIONS_SMALL, "Small")
    bench_jsonschema_compiled("jsonschema_validate_small", "SMALL_INSTANCE_VALID", ITERATIONS_SMALL, "Small")
    bench_jsonschema_not_compiled("SMALL_SCHEMA", "SMALL_INSTANCE_VALID", ITERATIONS_SMALL, "Small")
    bench_fastjsonschema_compiled("fast_validate_small", "SMALL_INSTANCE_VALID", ITERATIONS_SMALL, "Small")
    bench_fastjsonschema_not_compiled("SMALL_SCHEMA", "SMALL_INSTANCE_VALID", ITERATIONS_SMALL, "Small")
    bench_object("BIG_SCHEMA", "BIG_INSTANCE", ITERATIONS_BIG, "Big  ")
    bench_jsonschema_compiled("jsonschema_validate_big", "BIG_INSTANCE", ITERATIONS_BIG, "Big  ")
    bench_jsonschema_not_compiled("BIG_SCHEMA", "BIG_INSTANCE", ITERATIONS_BIG, "Big  ")
    bench_fastjsonschema_compiled("fast_validate_big", "BIG_INSTANCE", ITERATIONS_BIG, "Big  ")
    bench_fastjsonschema_not_compiled("BIG_SCHEMA", "BIG_INSTANCE", ITERATIONS_BIG, "Big  ")
