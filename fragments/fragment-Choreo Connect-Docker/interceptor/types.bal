// AUTO-GENERATED FILE.
// This file is auto-generated by the Ballerina OpenAPI tool.
import ballerina/http;

public type OkRequestHandlerResponseBody record {|
    *http:Ok;
    RequestHandlerResponseBody body;
|};

public type OkResponseHandlerResponseBody record {|
    *http:Ok;
    ResponseHandlerResponseBody body;
|};

public type ResponseHandlerResponseBody record {
    int responseCode?;
    # Map (string-to-string dictionary) of key value pairs of headers
    Headers headersToAdd?;
    # Map (string-to-string dictionary) of key value pairs of headers
    Headers headersToReplace?;
    # Array of header keys
    HeaderKeys headersToRemove?;
    # Map (string-to-string dictionary) of key value pairs of trailers
    Trailers trailersToAdd?;
    # Map (string-to-string dictionary) of key value pairs of trailers
    Trailers trailersToReplace?;
    # Array of header keys
    HeaderKeys trailersToRemove?;
    # Base64 encoded body
    Body body?;
};

public type DynamicEndpoint record {
    string endpointName?;
};

# Map (string-to-string dictionary) of key value pairs of headers
public type Headers record {|
    string...;
|};

public type InvocationContext_authenticationContext record {
    string tokenType?;
    string token?;
    string keyType?;
};

# Map (string-to-string dictionary) of key value pairs
public type InterceptorContext record {|
    string...;
|};

# Map (string-to-string dictionary) of key value pairs of trailers
public type Trailers record {|
    string...;
|};

public type RequestHandlerResponseBody record {
    boolean directRespond?;
    int responseCode?;
    DynamicEndpoint dynamicEndpoint?;
    # Map (string-to-string dictionary) of key value pairs of headers
    Headers headersToAdd?;
    # Map (string-to-string dictionary) of key value pairs of headers
    Headers headersToReplace?;
    # Array of header keys
    HeaderKeys headersToRemove?;
    # Map (string-to-string dictionary) of key value pairs of trailers
    Trailers trailersToAdd?;
    # Map (string-to-string dictionary) of key value pairs of trailers
    Trailers trailersToReplace?;
    # Array of header keys
    HeaderKeys trailersToRemove?;
    # Base64 encoded body
    Body body?;
    # Map (string-to-string dictionary) of key value pairs
    InterceptorContext interceptorContext?;
};

public type RequestHandlerRequestBody record {
    # Map (string-to-string dictionary) of key value pairs of headers
    Headers requestHeaders?;
    # Map (string-to-string dictionary) of key value pairs of trailers
    Trailers requestTrailers?;
    # Base64 encoded body
    Body requestBody?;
    InvocationContext invocationContext?;
};

public type InvocationContext record {
    string requestId?;
    string protocol?;
    string scheme?;
    string apiName?;
    string apiVersion?;
    string vhost?;
    string supportedMethods?;
    string method?;
    string basePath?;
    string path?;
    string pathTemplate?;
    string 'source?;
    string prodClusterName?;
    string sandClusterName?;
    InvocationContext_authenticationContext authenticationContext?;
};

# Array of header keys
public type HeaderKeys string[];

# Base64 encoded body
public type Body string;

public type ResponseHandlerRequestBody record {
    int responseCode;
    # Map (string-to-string dictionary) of key value pairs of headers
    Headers requestHeaders?;
    # Map (string-to-string dictionary) of key value pairs of trailers
    Trailers requestTrailers?;
    # Base64 encoded body
    Body requestBody?;
    # Map (string-to-string dictionary) of key value pairs of headers
    Headers responseHeaders?;
    # Map (string-to-string dictionary) of key value pairs of trailers
    Trailers responseTrailers?;
    # Base64 encoded body
    Body responseBody?;
    InvocationContext invocationContext?;
    # Map (string-to-string dictionary) of key value pairs
    InterceptorContext interceptorContext?;
};