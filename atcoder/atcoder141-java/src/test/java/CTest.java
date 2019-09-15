import static org.assertj.core.api.Assertions.assertThat;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class CTest {

    @Test
    void test() {
        final int[] res = C.calc(6, 3, 4, new int[] { 3, 1, 3, 2 });
        System.out.println(res);
    }

}
