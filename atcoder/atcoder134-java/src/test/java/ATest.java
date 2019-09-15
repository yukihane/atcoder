import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

public class ATest {

    @Test
    void test1() {
        assertThat(A.calc(4)).isEqualTo(48);
    }

}
