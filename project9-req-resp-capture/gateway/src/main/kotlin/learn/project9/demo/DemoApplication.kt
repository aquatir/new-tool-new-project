package learn.project9.demo

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.cloud.gateway.filter.factory.rewrite.RewriteFunction
import org.springframework.cloud.gateway.route.RouteLocator
import org.springframework.cloud.gateway.route.builder.RouteLocatorBuilder
import org.springframework.cloud.gateway.route.builder.filters
import org.springframework.cloud.gateway.route.builder.routes
import org.springframework.context.annotation.Bean
import reactor.core.publisher.Mono
import java.util.UUID

@SpringBootApplication
class DemoApplication {
    @Bean
    fun customRouteLocator(builder: RouteLocatorBuilder): RouteLocator {
        return builder.routes {
            route("root") {
                path("/")
                filters {
                    addResponseHeader("X-Custom-Header", "RootRouteHeader")
                }
                uri("localhost:8081")
            }
            route("kek") {
                path("/kek")
                filters {
                    addResponseHeader("X-Custom-Header", "KekRouterHeader")
                    addResponseHeader("X-Debug-Info", "Gateway successfully routed request to localhost:8081")
                }
                uri("localhost:8081/kek")
            }
            route("httpbin-get") {
                path("/get")
                filters {
                    addRequestHeader("Hello", "World")
                    modifyRequestBody(
                        String::class.java, String::class.java
                    ) { webExchange, originalBody ->
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
                    modifyResponseBody(String::class.java, String::class.java,
                        RewriteFunction { webExchange, originalBody ->
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
                    )
                }
                uri("http://httpbin.org:80")
            }

            route("httpbin-post") {
                path("/post")
                filters {
                    addRequestHeader("Hello", "World")
                    modifyRequestBody(
                        String::class.java, String::class.java
                    ) { webExchange, originalBody ->
                        val sb = StringBuilder()
                        val reqId = UUID.randomUUID()
                        webExchange.attributes["req-id"] = reqId
                        sb.append("RequestId '$reqId' Request ${webExchange.request.method} ${webExchange.request.uri}")
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
                    modifyResponseBody(String::class.java, String::class.java,
                        RewriteFunction { webExchange, originalBody ->
                            val sb = StringBuilder()
                            sb.append("RequestId '${webExchange.attributes["req-id"]}' Response ${webExchange.response.statusCode}")
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
                    )
                }
                uri("http://httpbin.org:80")
            }
        }

    }
}

fun main(args: Array<String>) {
    runApplication<DemoApplication>(*args)
}
