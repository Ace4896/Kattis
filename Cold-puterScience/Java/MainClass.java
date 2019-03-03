import java.util.Scanner;

public class MainClass {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int temperatureCount = sc.nextInt();
    int tempsBelowZero = 0;
    
    for (int i = 0; i < temperatureCount; i++) {
      if (sc.nextInt() < 0) {
        tempsBelowZero++;
      }
    }
    
    System.out.printf("%d\n", tempsBelowZero);
  }
}
