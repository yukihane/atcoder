import static org.assertj.core.api.Assertions.assertThat;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class BTest {

    @Test
    void test1() {
        assertThat(B.calc(6, 2)).isEqualTo(2);
    }

    @Test
    void test2() {
        assertThat(B.calc(14, 3)).isEqualTo(2);
    }

    @Test
    void test3() {
        assertThat(B.calc(20, 4)).isEqualTo(3);
    }

}
