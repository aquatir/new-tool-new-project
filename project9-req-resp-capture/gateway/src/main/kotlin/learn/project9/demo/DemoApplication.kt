package learn.project9.demo

import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.cloud.gateway.route.RouteLocator
import org.springframework.cloud.gateway.route.builder.GatewayFilterSpec
import org.springframework.cloud.gateway.route.builder.RouteLocatorBuilder
import org.springframework.cloud.gateway.route.builder.filters
import org.springframework.cloud.gateway.route.builder.routes
import org.springframework.context.annotation.Bean
import org.springframework.http.HttpMethod
import org.springframework.web.server.ServerWebExchange
import reactor.core.publisher.Mono
import java.net.URI
import java.util.UUID

const val REQ_ID = "req-id"

data class RequestData(
    val reqId: String,
    val method: HttpMethod,
    val uri: URI,
    val headers: Map<String, List<String>>,
    var body: String? = null
)

data class ResponseData(
    val reqId: String?, val headers: Map<String, List<String>>, var body: String? = null
)

@SpringBootApplication
class DemoApplication {
    @Bean
    fun customRouteLocator(builder: RouteLocatorBuilder, mappers: Mappers): RouteLocator {
        val requestBodyMapper = mappers.requestBodyMapper()
        val responseBodyMapper = mappers.responseBodyMapper()
        return builder.routes {
            route("httpbin") {
                path("/httpbin/**")
                filters {
                    rewritePath("/httpbin/(?<segment>.*)", "/\${segment}")
                    modifyRequestBody(String::class.java, String::class.java, requestBodyMapper)
                    modifyResponseBody(String::class.java, String::class.java, responseBodyMapper)
                }
                uri("http://httpbin.org")
            }
        }
    }
}

fun main(args: Array<String>) {
    runApplication<DemoApplication>(*args)
}
