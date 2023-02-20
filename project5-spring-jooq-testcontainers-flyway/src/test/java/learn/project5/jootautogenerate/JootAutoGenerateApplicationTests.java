package learn.project5.jootautogenerate;

import org.jooq.DSLContext;
import org.jooq.generated.Tables;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;

import java.util.UUID;

import static org.jooq.generated.tables.Customers.CUSTOMERS;

@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.NONE)
class JootAutoGenerateApplicationTests {

    @Autowired
    private DSLContext dslContext;

    @Test
    void contextLoads() {

        dslContext.insertInto(Tables.CUSTOMERS)
                .set(CUSTOMERS.ID, UUID.randomUUID())
                .set(CUSTOMERS.FIRST_NAME, "foo")
                .set(CUSTOMERS.LAST_NAME, "bar")
                .execute();

        dslContext.selectFrom(Tables.CUSTOMERS)
                .fetch().forEach(it -> System.out.println("%s: %s %s".formatted(
                        it.getId(), it.getFirstName(), it.getLastName()
                )));
    }

}
