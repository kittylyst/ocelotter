public class AddFieldInteger {

    public static int main2(String[] args) {
        Integer i7 = 7;
        Integer i4 = 4;
        FieldHaver fh = new FieldHaver();
        fh.i = 9;
        return i7 + i4 + fh.i;
    }

}