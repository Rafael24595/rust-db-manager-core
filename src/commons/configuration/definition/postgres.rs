use serde_json::json;

//TODO: Reconsider configuration logic.
pub fn postgres_collection() -> String {
    json!(
        {
            "swrelational": true,
            "definition": [
                {
                    "order": 1,
                    "name": "Small Integer",
                    "code": "SMALLINT",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 2,
                    "name": "Integer",
                    "code": "INTEGER",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 3,
                    "name": "Big Integer",
                    "code": "BIGINT",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 4,
                    "name": "Decimal",
                    "code": "DECIMAL",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 5,
                    "name": "Numeric",
                    "code": "NUMERIC",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 6,
                    "name": "Real",
                    "code": "REAL",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 7,
                    "name": "Double Precision",
                    "code": "DOUBLE PRECISION",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 8,
                    "name": "Serial",
                    "code": "SERIAL",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 9,
                    "name": "BigSerial",
                    "code": "BIGSERIAL",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 10,
                    "name": "Character",
                    "code": "CHAR",
                    "swkey": true,
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 11,
                    "name": "Varchar",
                    "code": "VARCHAR",
                    "swkey": true,
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 12,
                    "name": "Text",
                    "code": "TEXT",
                    "swkey": true,
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 13,
                    "name": "Binary Data",
                    "code": "BYTEA",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 14,
                    "name": "Date",
                    "code": "DATE",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 15,
                    "name": "Time",
                    "code": "TIME",
                    "swkey": true,
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 16,
                    "name": "Timestamp",
                    "code": "TIMESTAMP",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 17,
                    "name": "Interval",
                    "code": "INTERVAL",
                    "swkey": true,
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 18,
                    "name": "Boolean",
                    "code": "BOOLEAN",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 19,
                    "name": "Enum",
                    "code": "ENUM",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 20,
                    "name": "Point",
                    "code": "POINT",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 21,
                    "name": "Line",
                    "code": "LINE",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 22,
                    "name": "Polygon",
                    "code": "POLYGON",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 23,
                    "name": "JSON",
                    "code": "JSON",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 24,
                    "name": "JSONB",
                    "code": "JSONB",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 25,
                    "name": "UUID",
                    "code": "UUID",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 26,
                    "name": "Array",
                    "code": "ARRAY",
                    "swkey": true,
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 27,
                    "name": "Range",
                    "code": "RANGE",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 28,
                    "name": "Text Search Vector",
                    "code": "TSVECTOR",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 29,
                    "name": "Text Search Query",
                    "code": "TSQUERY",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 30,
                    "name": "XML",
                    "code": "XML",
                    "swkey": true,
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                }
            ],
            "defaults": [
                {
                    "order": 0,
                    "code": "SERIAL",
                    "value": "_id",
                    "swkey": true,
                    "swsize": false,
                    "size": 0,
                    "mutable": false,
                    "json_type": "NUMERIC",
                    "attributes": [],
                    "reference": []
                }
            ]
        }
    ).to_string()
}

pub fn postgres_collection_actions() -> String {
    json!(
        [
            
        ]
    ).to_string()
}
