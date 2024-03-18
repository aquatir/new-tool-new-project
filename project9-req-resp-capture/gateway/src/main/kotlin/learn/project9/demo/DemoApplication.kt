package learn.project9.demo

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.cloud.gateway.route.RouteLocator
import org.springframework.cloud.gateway.route.builder.RouteLocatorBuilder
import org.springframework.cloud.gateway.route.builder.filters
import org.springframework.cloud.gateway.route.builder.routes
import org.springframework.context.annotation.Bean
import org.springframework.web.server.ServerWebExchange
import reactor.core.publisher.Mono

val requestBodyMapper: (ServerWebExchange, String) -> Mono<String> =
    { webExchange: ServerWebExchange, originalBody: String? ->
        val sb = StringBuilder()
        sb.append("Request ${webExchange.request.method} ${webExchange.request.uri}")
        sb.append(" Headers: ")
        webExchange.request.headers.forEach { key, value -> sb.append("$key: $value, ") }
        val returnBody = if (originalBody != null) {
            sb.append(" Body: $originalBody")
            Mono.just(originalBody)
        } else {
            Mono.empty()
        }
        println(sb)
        returnBody
    }

val responseBodyMapper: (ServerWebExchange, String) -> Mono<String> =
    { webExchange: ServerWebExchange, originalBody: String? ->
        val sb = StringBuilder()
        sb.append("Response ${webExchange.response.statusCode}")
        sb.append(" Headers: ")
        webExchange.response.headers.forEach { key, value -> sb.append("$key: $value, ") }
        val returnBody = if (originalBody != null) {
            sb.append(" Body: $originalBody")
            Mono.just(originalBody)
        } else {
            Mono.empty()
        }
        println(sb)
        returnBody
    }

@SpringBootApplication
class DemoApplication {
    @Bean
    fun customRouteLocator(builder: RouteLocatorBuilder): RouteLocator {
        return builder.routes {
            route("httpbin") {
                path("/httpbin/**")
                filters {
                    rewritePath("/httpbin/(?<segment>.*)", "/\${segment}")
                    addRequestHeader("Hello", "World")
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
