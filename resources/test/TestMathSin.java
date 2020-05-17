public class TestMathSin {

    public static int main_ifge() {
        double d = Math.sin(0.5);
        if (d < 0.5) {
            return 1;
        }
        return 0;
    }

    public static int main_ifle() {
        double d = Math.sin(0.5);
        if (d > 0.5) {
            return 1;
        }
        return 0;
    }


    public static int main_ifnull() {
        Object o = null;
        if (o != null) {
            return 1;
        }
        return 0;
    }

}