import java.util.*;

public class Main {
    public static void main(final String[] args) {
        final Scanner sc = new Scanner(System.in);
        // 整数の入力
        final int a = sc.nextInt();
        // スペース区切りの整数の入力
        final int b = sc.nextInt();
        final int c = sc.nextInt();
        // 文字列の入力
        final String s = sc.next();
        // 出力
        System.out.println((a + b + c) + " " + s);
    }
}
