package learn.project9.demo

import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.cloud.gateway.route.builder.GatewayFilterSpec
import org.springframework.http.HttpMethod
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

@Component
class Filters(val objectMapper: ObjectMapper) {

    fun requestResponseLoggingFilter(myGatewayFilter: GatewayFilterSpec) {
        myGatewayFilter.modifyRequestBody(String::class.java, String::class.java, requestBodyMapper)
        myGatewayFilter.modifyResponseBody(String::class.java, String::class.java, responseBodyMapper)

        // Remove all response headers starting with X-TG
        myGatewayFilter.filter { exchange, chain ->
            chain.filter(exchange)
                .then(Mono.fromRunnable {
                    exchange.response.headers.filter { it.key.startsWith("X-TG") }
                        .forEach {
                            exchange.response.headers.remove(it.key)
                        }

                });
        }
    }

    private val requestBodyMapper: (ServerWebExchange, String?) -> Mono<String?> =
        { webExchange: ServerWebExchange, originalBody: String? ->
            val reqData = RequestData(
                reqId = UUID.randomUUID().toString().also {
                    webExchange.attributes[REQ_ID] = it
                },
                method = webExchange.request.method,
                uri = webExchange.request.uri,
                headers = webExchange.request.headers
            )

            val returnBody = if (originalBody != null) {
                reqData.body = tryParseJson(originalBody)
                Mono.just(originalBody)
            } else {
                Mono.empty()
            }
            println(reqData)
            returnBody
        }

    private val responseBodyMapper: (ServerWebExchange, String?) -> Mono<String?> =
        { webExchange: ServerWebExchange, originalBody: String? ->
            val respData = ResponseData(
                reqId = webExchange.attributes[REQ_ID] as String?,
                status = webExchange.response.statusCode?.value(),
                headers = webExchange.response.headers
            )
            val returnBody = if (originalBody != null) {
                respData.body = tryParseJson(originalBody)
                Mono.just(originalBody)
            } else {
                Mono.empty()
            }
            println(respData)
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
}
