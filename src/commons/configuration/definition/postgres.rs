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
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 2,
                    "name": "Integer",
                    "code": "INTEGER",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 3,
                    "name": "Big Integer",
                    "code": "BIGINT",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 4,
                    "name": "Decimal",
                    "code": "DECIMAL",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 5,
                    "name": "Numeric",
                    "code": "NUMERIC",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 6,
                    "name": "Real",
                    "code": "REAL",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 7,
                    "name": "Double Precision",
                    "code": "DOUBLE PRECISION",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 8,
                    "name": "Serial",
                    "code": "SERIAL",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 9,
                    "name": "BigSerial",
                    "code": "BIGSERIAL",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 10,
                    "name": "Character",
                    "code": "CHAR",
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 11,
                    "name": "Varchar",
                    "code": "VARCHAR",
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 12,
                    "name": "Text",
                    "code": "TEXT",
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 13,
                    "name": "Binary Data",
                    "code": "BYTEA",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 14,
                    "name": "Date",
                    "code": "DATE",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 15,
                    "name": "Time",
                    "code": "TIME",
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 16,
                    "name": "Timestamp",
                    "code": "TIMESTAMP",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 17,
                    "name": "Interval",
                    "code": "INTERVAL",
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 18,
                    "name": "Boolean",
                    "code": "BOOLEAN",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 19,
                    "name": "Enum",
                    "code": "ENUM",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 20,
                    "name": "Point",
                    "code": "POINT",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 21,
                    "name": "Line",
                    "code": "LINE",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 22,
                    "name": "Polygon",
                    "code": "POLYGON",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 23,
                    "name": "JSON",
                    "code": "JSON",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 24,
                    "name": "JSONB",
                    "code": "JSONB",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 25,
                    "name": "UUID",
                    "code": "UUID",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 26,
                    "name": "Array",
                    "code": "ARRAY",
                    "swsize": true,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 27,
                    "name": "Range",
                    "code": "RANGE",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 28,
                    "name": "Text Search Vector",
                    "code": "TSVECTOR",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 29,
                    "name": "Text Search Query",
                    "code": "TSQUERY",
                    "swsize": false,
                    "multiple": true,
                    "attributes": []
                },
                {
                    "order": 30,
                    "name": "XML",
                    "code": "XML",
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
                    "swsize": false,
                    "size": 0,
                    "mutable": false,
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
