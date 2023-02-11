package learn.project5.jootautogenerate;

import org.jooq.DSLContext;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;

@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.NONE)
class JootAutoGenerateApplicationTests {

    @Autowired
    private DSLContext dslContext;

	@Test
	void contextLoads() {

	}

}
