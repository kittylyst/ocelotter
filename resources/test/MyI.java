public class MyI {
    private int i;

    public int getI() {
        return i;
    }

    public void setI(int j) {
        i = j;
    }

    public static int main2(String[] args) {
        MyI o = new MyI();
        o.setI(42);
        return o.getI();
    }
}