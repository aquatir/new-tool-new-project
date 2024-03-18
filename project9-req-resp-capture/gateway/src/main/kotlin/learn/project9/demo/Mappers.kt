package learn.project9.demo

import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.stereotype.Component
import org.springframework.web.server.ServerWebExchange
import reactor.core.publisher.Mono
import java.util.UUID

@Component
class Mappers(val objectMapper: ObjectMapper) {

    fun requestBodyMapper(): (ServerWebExchange, String) -> Mono<String> =
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

    fun responseBodyMapper(): (ServerWebExchange, String) -> Mono<String> =
        { webExchange: ServerWebExchange, originalBody: String? ->
            val respData = ResponseData(
                reqId = webExchange.attributes[REQ_ID] as String?, headers = webExchange.response.headers
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
