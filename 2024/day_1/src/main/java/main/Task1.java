package main;

import java.io.File;
import java.util.ArrayList;
import java.util.Scanner;

public class Task1 {

    public record Pair<T, U>(T num, U index) {

    }

    public static void main(String[] args) {
        try {
            File file = new File("./input.txt");

            ArrayList<Integer> array1 = new ArrayList<>();
            ArrayList<Integer> array2 = new ArrayList<>();

            try (Scanner reader = new Scanner(file)) {
                while (reader.hasNextLine()) {
                    String line = reader.nextLine();
                    String[] parts = line.split("   ");
                    array1.add(Integer.parseInt(parts[0]));
                    array2.add(Integer.parseInt(parts[1]));
                }
            }

            array1.sort((a, b) -> a - b);
            array2.sort((a, b) -> a - b);

            int total_distance = 0;

            for (int i = 0; i < array1.size(); i++) {
                Integer num_1 = array1.get(i);
                Integer num_2 = array2.get(i);

                int distance = Math.abs(num_1 - num_2);
                System.err.println(distance);
                total_distance += distance;
            }

            System.err.println(total_distance);
        } catch (Exception e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }
}
