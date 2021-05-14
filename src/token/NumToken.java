package token;

/**
 * @author Zexho
 * @date 2021/5/14 5:13 下午
 */
public class NumToken extends Token {
    private final int value;

    public NumToken(int line, int v) {
        super(line);
        this.value = v;
    }

    @Override
    public boolean isNumber() {
        return true;
    }

    @Override
    public String getText() {
        return Integer.toString(this.value);
    }

    @Override
    public int getNumber() {
        return value;
    }
}
