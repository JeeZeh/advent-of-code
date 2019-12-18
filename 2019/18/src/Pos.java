public class Pos {
    int x;
    int y;

    public Pos(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public Pos add(int a, int b) {
        this.x+=a;
        this.y+=b;
        return this;
    }
}
