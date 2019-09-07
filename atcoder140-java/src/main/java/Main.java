import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Objects;
import java.util.Scanner;

public class Main {
    public static void main(final String[] args) {
        final int p = 1;
        switch (p) {
        case 1:
            A.answer();
            break;
        }
    }
}

class A {
    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final List<LinkedList<Integer>> table = new ArrayList<>(n);
        for (int i = 0; i < n; i++) {
            final LinkedList<Integer> ll = new LinkedList<>();
            table.add(ll);
            for (int j = 0; j < n - 1; j++) {
                final int x = sc.nextInt();
                ll.add(x);
            }
        }
        final int res = calc(table);
        System.out.println(res);
    }

    static int calc(final List<LinkedList<Integer>> table) {
        // TODO Auto-generated method stub
        return 0;
    }
}