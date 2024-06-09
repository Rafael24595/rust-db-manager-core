use serde_json::json;

//TODO: Reconsider configuration logic.
pub fn mongo_db_collection() -> String {
    json!(
        {
            "swrelational": false,
            "definition": [
                {
                    "order": 0,
                    "name": "Index",
                    "code": "INDEXED",
                    "swsize": false,
                    "multiple": true,
                    "attributes": [
                        {
                            "name": "Unique",
                            "code": "UNIQUE",
                            "values": [
                                {
                                    "key": "True",
                                    "value": "true"
                                },
                                {
                                    "key": "False",
                                    "value": "false"
                                }
                            ]
                        },
                        {
                            "name": "Direction",
                            "code": "DIRECTION",
                            "values": [
                                {
                                    "key": "ASC",
                                    "value": "1"
                                },
                                {
                                    "key": "DESC",
                                    "value": "-1"
                                }
                            ]
                        }
                    ]
                }
            ],
            "defaults": [
                {
                    "order": 0,
                    "code": "INDEXED",
                    "value": "_id",
                    "swsize": false,
                    "size": 0,
                    "mutable": false,
                    "attributes": [
                        {
                            "key": "UNIQUE",
                            "value": "true"
                        },
                        {
                            "key": "DIRECTION",
                            "value": "1"
                        }
                    ],
                    "reference": []
                }
            ]
        }
    ).to_string()
}

pub fn mongo_db_filter() -> String {
    json!(
        {
            "attributes": [
                {
                    "code": "OID",
                    "name": "ObjectID",
                    "description": "ObjectID",
                    "values": [
                        {
                            "key": "True",
                            "value": "true",
                            "default": false
                        },
                        {
                            "key": "False",
                            "value": "false",
                            "default": true
                        }
                    ],
                    "applies": [
                        "ID_STRING",
                    ]
                },
                {
                    "code": "REGEX",
                    "name": "Regex",
                    "description": "Filter contains",
                    "values": [
                        {
                            "key": "True",
                            "value": "true",
                            "default": false
                        },
                        {
                            "key": "False",
                            "value": "false",
                            "default": true
                        }
                    ],
                    "applies": [
                        "STRING"
                    ]
                },
            ]
        }
    ).to_string()
}