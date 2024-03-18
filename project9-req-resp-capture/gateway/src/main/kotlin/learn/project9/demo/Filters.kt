package learn.project9.demo

import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.cloud.gateway.route.builder.GatewayFilterSpec
import org.springframework.http.HttpHeaders
import org.springframework.http.HttpMessage
import org.springframework.http.HttpMethod
import org.springframework.scheduling.annotation.Async
import org.springframework.stereotype.Component
import org.springframework.web.server.ServerWebExchange
import reactor.core.publisher.Mono
import java.net.URI
import java.util.UUID

data class RequestData(
    val reqId: String,
    val method: HttpMethod,
    val uri: URI,
    val headers: Map<String, List<String>>,
    var body: String? = null
)

data class ResponseData(
    val reqId: String?,
    val status: Int?,
    val headers: Map<String, List<String>>,
    var body: String? = null
)

private fun HttpHeaders.deepCopy(): Map<String, List<String>> = HashMap(this)

@Component
class Filters(val objectMapper: ObjectMapper) {

    fun requestResponseLoggingFilter(myGatewayFilter: GatewayFilterSpec) {
        myGatewayFilter.modifyRequestBody(String::class.java, String::class.java, requestMapper)
        myGatewayFilter.modifyResponseBody(String::class.java, String::class.java, responseMapper)
    }

    private val requestMapper: (ServerWebExchange, String?) -> Mono<String?> =
        { exchange: ServerWebExchange, originalBody: String? ->
            val reqData = RequestData(
                reqId = UUID.randomUUID().toString().also {
                    exchange.attributes[REQ_ID] = it
                },
                method = exchange.request.method,
                uri = exchange.request.uri,
                headers = exchange.request.headers.deepCopy()
            )

            val returnBody = if (originalBody != null) {
                reqData.body = tryParseJson(originalBody)
                Mono.just(originalBody)
            } else {
                Mono.empty()
            }
            logRequestData(reqData)
            returnBody
        }

    private val responseMapper: (ServerWebExchange, String?) -> Mono<String?> =
        { exchange: ServerWebExchange, originalBody: String? ->
            val respData = ResponseData(
                reqId = exchange.attributes[REQ_ID] as String?,
                status = exchange.response.statusCode?.value(),
                headers = exchange.response.headers.deepCopy()
            )

            // remove headers starting with X-TG
            exchange.response.headers.filter { it.key.startsWith("X-TG") }
                .forEach {
                    exchange.response.headers.remove(it.key)
                }

            val returnBody = if (originalBody != null) {
                respData.body = tryParseJson(originalBody)
                Mono.just(originalBody)
            } else {
                Mono.empty()
            }
            logResponseData(respData)
            returnBody
        }

    private fun tryParseJson(input: String): String? {
        return try {
            val jsonNode = objectMapper.readTree(input)
            return objectMapper.writeValueAsString(jsonNode)
        } catch (e: Exception) {
            null
        }
    }

    @Async
    fun logRequestData(requestData: RequestData) = println(requestData)

    @Async
    fun logResponseData(responseData: ResponseData) = println(responseData)

}


