package learn.project9.demo

import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.cloud.gateway.route.builder.GatewayFilterSpec
import org.springframework.http.HttpHeaders
import org.springframework.kafka.core.KafkaTemplate
import org.springframework.scheduling.annotation.Async
import org.springframework.stereotype.Component
import org.springframework.web.server.ServerWebExchange
import reactor.core.publisher.Mono
import java.time.OffsetDateTime
import java.util.UUID

data class RequestData(
    val reqId: String,
    val method: String,
    val uri: String,
    val headers: Map<String, List<String>>,
    val timestamp: OffsetDateTime = OffsetDateTime.now(),
    var body: JsonNode? = null,
)

data class ResponseData(
    val reqId: String?,
    val status: Int?,
    val headers: Map<String, List<String>>,
    val timestamp: OffsetDateTime = OffsetDateTime.now(),
    var body: JsonNode? = null,
)

private fun HttpHeaders.deepCopy(): Map<String, List<String>> = HashMap(this)

@Component
class Filters(
    val objectMapper: ObjectMapper,
    val kafkaTemplate: KafkaTemplate<String, String>,
) {

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
                method = exchange.request.method.name(),
                uri = exchange.request.uri.toString(),
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

            // remove headers starting with X-CompanyName
            exchange.response.headers.filter { it.key.startsWith("X-CompanyName") }
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

    private fun tryParseJson(input: String): JsonNode? {
        return try {
            return objectMapper.readTree(input)
        } catch (e: Exception) {
            null
        }
    }

    @Async
    fun logRequestData(requestData: RequestData) {

        val dataAsJson = objectMapper.writeValueAsString(requestData)
        kafkaTemplate.send("topic", dataAsJson)
        println(dataAsJson)
    }

    @Async
    fun logResponseData(responseData: ResponseData) {
        val dataAsJson = objectMapper.writeValueAsString(responseData)
        kafkaTemplate.send("topic", dataAsJson)
        println(dataAsJson)
    }

}


