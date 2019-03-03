import java.util.Scanner;

public class MainClass {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    double cost = sc.nextDouble();
    int lawnsToSow = sc.nextInt();
    
    double finalCost = 0.0f;
    
    for (int i = 0; i < lawnsToSow; i++) {
      double width = sc.nextDouble();
      double length = sc.nextDouble();
      
      finalCost += width * length * cost;
    }
    
    System.out.printf("%f\n", finalCost);
  }
}
