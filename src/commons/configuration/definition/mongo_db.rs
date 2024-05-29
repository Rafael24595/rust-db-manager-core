use serde_json::json;

//TODO: Reconsider configuration logic.
pub fn mongo_db() -> String {
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
