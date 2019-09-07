import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Objects;
import java.util.Scanner;

public class Main {
    public static void main(final String[] args) {
        final int p = 5;
        switch (p) {
        case 5:
            P5.answer();
            break;
        case 4:
            P4.answer();
            break;
        case 3:
            P3.answer();
            break;
        case 2:
            P2.answer();
            break;
        case 1:
            P1.p1();
            break;
        }
    }
}

class P5 {
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
        int days = 0;
        while (!endGame(table)) {
            final boolean res = todayMatch(table);
            if (res) {
                days++;
            } else {
                return -1;
            }
        }
        return days;
    }

    static boolean endGame(final List<LinkedList<Integer>> table) {
        for (final LinkedList<Integer> l : table) {
            if (!l.isEmpty()) {
                return false;
            }
        }
        return true;
    }

    static boolean todayMatch(final List<LinkedList<Integer>> table) {
        final ArrayList<LinkedList<Integer>> todayTable = new ArrayList<>(table);
        int myNumber = 0;
        boolean haveGame = false;
        while (myNumber <= todayTable.size()) {
            for (int i = myNumber; i < table.size(); i++) {
                final LinkedList<Integer> mySchedule = todayTable.get(0);
                final Integer myOpposition = mySchedule.pop();
                final LinkedList<Integer> oSchedule = table.get(myOpposition.intValue() - 1);
                final Integer oo = oSchedule.pop();
                if (oo.intValue() == i) {
                    todayTable.remove(myOpposition.intValue() - 1);
                    todayTable.ensureCapacity(i);
                    myNumber = i;
                    haveGame = true;
                    break;
                } else {
                    mySchedule.push(myOpposition);
                    oSchedule.push(oo);
                }
            }
        }
        return haveGame;
    }
}

class P4 {
    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int res = calc(n);
        System.out.println(res);
    }

    static int calc(final int n) {
        int sum = 0;
        for (int i = 1; i <= n; i++) {
            sum += i - 1;
        }
        return sum;
    }
}

class P3 {
    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int n = sc.nextInt();
        final int[] array = new int[n];
        for (int i = 0; i < n; i++) {
            array[i] = sc.nextInt();
        }
        final int res = calc(array);
        System.out.println(res);
    }

    static int calc(final int[] array) {
        int max = 0;
        for (int i = 0; i < array.length; i++) {
            if (max > array.length - i) {
                break;
            }
            final int res = measure(array, i);
            max = Math.max(res, max);
        }
        return max;
    }

    private static int measure(final int[] array, final int pos) {
        int times = 0;
        for (int i = pos; i < array.length - 1; i++) {
            if (array[i] >= array[i + 1]) {
                times++;
            } else {
                break;
            }
        }
        return times;
    }

}

class P2 {
    static void answer() {
        final Scanner sc = new Scanner(System.in);
        final int a = sc.nextInt();
        final int b = sc.nextInt();
        final int res = calc(a, b);
        System.out.println(res);
    }

    static int calc(final int a, final int b) {
        final int per = a - 1;
        final int remain = b - 1;
        final int x = remain / per;
        if (remain - (per * x) > 0) {
            return x + 1;
        }
        return x;
    }
}

class P1 {

    static void p1() {
        final Scanner sc = new Scanner(System.in);
        final String yohou = sc.nextLine();
        final String jissai = sc.nextLine();

        final int res = calc(yohou, jissai);
        System.out.println(res);
    }

    static int calc(final String yohou, final String kekka) {
        int res = 0;
        for (int i = 0; i < 3; i++) {
            if (Objects.equals(yohou.charAt(i), kekka.charAt(i))) {
                res++;
            }
        }
        return res;
    }
}