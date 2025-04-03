use std::borrow::Cow;

/// A rpc call is represented by sending a Request object to a Server.
pub struct JsonRPCRequestObject {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    jsonrpc: Cow<'static, str>,
    /// An identifier established by the Client that MUST contain a String, Number, or NULL value if included.
    /// If it is not included it is assumed to be a notification.
    /// The value SHOULD normally not be Null \[1\] and Numbers SHOULD NOT contain fractional parts \[2\]
    ///
    /// The Server MUST reply with the same value in the Response object if included.
    /// This member is used to correlate the context between the two objects.
    ///
    /// \[1\] The use of Null as a value for the id member in a Request object is discouraged,
    /// because this specification uses a value of Null for Responses with an unknown id.
    /// Also, because JSON-RPC 1.0 uses an id value of Null for Notifications this could cause confusion in handling.
    ///
    /// \[2\] Fractional parts may be problematic, since many decimal fractions cannot be represented exactly as binary fractions.
    id: Option<Id>,
    /// A String containing the name of the method to be invoked.
    /// Method names that begin with the word rpc followed by a period character (U+002E or ASCII 46)
    /// are reserved for rpc-internal methods and extensions and MUST NOT be used for anything else.
    method: Option<String>,
    /// A Structured value that holds the parameter values to be used during the invocation of the method. This member MAY be omitted.
    params: Option<String>,
}

/// An identifier established by the Client that MUST contain a String, Number, or NULL value if included.
/// If it is not included it is assumed to be a notification.
/// The value SHOULD normally not be Null \[1\] and Numbers SHOULD NOT contain fractional parts \[2\]
///
/// The Server MUST reply with the same value in the Response object if included.
/// This member is used to correlate the context between the two objects.
///
/// \[1\] The use of Null as a value for the id member in a Request object is discouraged,
/// because this specification uses a value of Null for Responses with an unknown id.
/// Also, because JSON-RPC 1.0 uses an id value of Null for Notifications this could cause confusion in handling.
///
/// \[2\] Fractional parts may be problematic, since many decimal fractions cannot be represented exactly as binary fractions.
enum Id {
    String(String),
    Number(u64),
}

/// | code             | message          | meaning                                                                                               |
/// |------------------|------------------|-------------------------------------------------------------------------------------------------------|
/// | -32700           | Parse error      | Invalid JSON was received by the server. An error occurred on the server while parsing the JSON text. |
/// | -32600           | Invalid Request  | The JSON sent is not a valid Request object.                                                          |
/// | -32601           | Method not found | The method does not exist / is not available.                                                         |
/// | -32602           | Invalid params   | Invalid method parameter(s).                                                                          |
/// | -32603           | Internal error   | Internal JSON-RPC error.                                                                              |
/// | -32000 to -32099 | Server error     | Reserved for implementation-defined server-errors.                                                    |
struct ErrorObject {
    code: i32,
    message: Cow<'static, str>,
    // TODO: type will be changed to Box<T>, serde raw.
    data: String,
}

impl ErrorObject {
    pub fn set(code: i32) {
        match code {
            -32700 => println!("Parse Error"),
            -32600 => println!("Invalid Error"),
            -32601 => println!("Method not found"),
            -32602 => println!("Invalid params"),
            -32603 => println!("Internal error"),
            -32099..=-32000 => println!("Server error"),
            _ => panic!("Undefined"),
        }
    }
}
