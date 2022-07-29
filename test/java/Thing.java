public class Thing implements java.io.Serializable {
    private Object x;
    public Thing(Object x) {
        this.x = x;
    }

    public Object getX() {
        return this.x;
    }

    public void setX(Object x) {
        this.x = x;
    }
}