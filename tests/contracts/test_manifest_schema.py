import json
import os
from jsonschema import validate, Draft7Validator

ROOT = os.path.join(os.path.dirname(__file__), '..', 'contracts')
SCHEMA_PATH = os.path.join(ROOT, 'manifest.schema.json')


def load(path):
    with open(path, 'r') as f:
        return json.load(f)


def test_manifest_local_valid():
    schema = load(SCHEMA_PATH)
    manifest = load(os.path.join(os.path.dirname(__file__), '..', 'specs', '001-spec-impl-annotations', 'examples', 'manifest.local.json'))
    Draft7Validator(schema).validate(manifest)


def test_manifest_pr_valid():
    schema = load(SCHEMA_PATH)
    manifest = load(os.path.join(os.path.dirname(__file__), '..', 'specs', '001-spec-impl-annotations', 'examples', 'manifest.pr.json'))
    Draft7Validator(schema).validate(manifest)


def test_manifest_broken_valid():
    schema = load(SCHEMA_PATH)
    manifest = load(os.path.join(os.path.dirname(__file__), '..', 'specs', '001-spec-impl-annotations', 'examples', 'manifest.broken.json'))
    Draft7Validator(schema).validate(manifest)
