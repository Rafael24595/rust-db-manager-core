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

pub fn mongo_db_collection_actions() -> String {
    json!(
        [
            {
                "action": "INDEXES_NEW",
                "title": "New indexes",
                "form": {
                    "sw_query": true,
                    "forms":  [
                        {
                            "sw_vector": true,
                            "fields": [
                                {
                                    "order": 1,
                                    "code": "FIELD",
                                    "name": "Field",
                                    "sw_vector": false,
                                    "values": []
                                },
                                {
                                    "order": 2,
                                    "code": "DIRECTION",
                                    "name": "Direction",
                                    "sw_vector": false,
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
                        },
                        {
                            "sw_vector": false,
                            "fields": [
                                {
                                    "order": 1,
                                    "code": "NAME",
                                    "name": "Name",
                                    "sw_vector": false,
                                    "values": []
                                },
                                {
                                    "order": 2,
                                    "code": "UNIQUE",
                                    "name": "Unique",
                                    "sw_vector": false,
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
                                }
                            ]
                        }
                    ]
                }
            }
        ]
    ).to_string()
}