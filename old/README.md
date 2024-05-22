# AutoGPT Server

To build:

```
cargo build --release --target x86_64-unknown-linux-gnu --target aarch64-apple-darwin --target x86_64-pc-windows-gnu
```




// All input sources are triggers

/agent/alpha/input/chat/message
/agent/alpha/input/file/upload
/agent/alpha/input/file/directory
/agent/alpha/input/video/stream
/agent/alpha/input/audio/stream


// In addition to input sources, there can be custom tiggers that route data to a input

/agent/alpha/trigger/reminder/22:00 -> /agent/alpha/input/chat/message
/agent/alpha/trigger/email -> /agent/alpha/input/chat/message


// Outputs must be mapped to some method of returning the data to the client
/agent/alpha/outputs/message
/agent/alpha/outputs/audio/stream
/agent/alpha/outputs/video/stream
/agent/alpha/outputs/file/directory
/agent/alpha/outputs/url



// Trigger Objects are used only for item that are not inputs.
// By default all inputs are mapped as triggers to the api provider.
TriggerType {
    ScheduledTask,
    CRON,
    Custom

}

Agent {
    template_id: str
    agent_id: str


    [

        Trigger {
            type: TriggerType.Custom,
            tigger_class: string // EmailListner class name of the trigger 
            data_format: schema, 
            input_target: messages-input-id // input must be present on the root component
            config: {}
        }
        ScheduledMessage {
            hook_name: string
            type: TriggerType.ScheduledTeask, // Builtin type so no need to map to a class
            data_format: schema, 
            input_target: messages-input-id // input must be present on the root component
            config: {
                timestamp
            }
        }

    ]

    [
        Caches {
            cache_id: str
            cache_type: CacheType
            key: string
            data_type: Type
            storage_amount: int
            TTL: int
        }
        
    ]

    [
        Component {
            id: string 
            parent_id: String | None
            ComponentClass: string
            linked_caches: [cache_ids]
            input_schema: [Inputs]
            output_schema: [Outputs]
            linked: List[ids]
        }
        
    ]
}