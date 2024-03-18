package learn.project9.demo

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.cloud.gateway.route.RouteLocator
import org.springframework.cloud.gateway.route.builder.RouteLocatorBuilder
import org.springframework.cloud.gateway.route.builder.filters
import org.springframework.cloud.gateway.route.builder.routes
import org.springframework.context.annotation.Bean
import org.springframework.http.HttpMethod
import java.net.URI

const val REQ_ID = "req-id"

@SpringBootApplication
class DemoApplication {
    @Bean
    fun customRouteLocator(builder: RouteLocatorBuilder, filters: Filters): RouteLocator {

        return builder.routes {
            route("httpbin") {
                host("localhost:8080")
                path("/httpbin/**")
                filters {
                    rewritePath("/httpbin/(?<segment>.*)", "/\${segment}")
                    filters.requestResponseLoggingFilter(this)
                }
                uri("http://httpbin.org")
            }

            route("backend") {
                host("localhost:8080")
                path("/backend/**")
                filters {
                    rewritePath("/backend/(?<segment>.*)", "/\${segment}")
                    filters.requestResponseLoggingFilter(this)
                }
                uri("http://localhost:8081")
            }
        }
    }
}

fun main(args: Array<String>) {
    runApplication<DemoApplication>(*args)
}
