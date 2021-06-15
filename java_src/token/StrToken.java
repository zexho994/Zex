package token;

/**
 * @author Zexho
 * @date 2021/5/14 5:17 下午
 */
public class StrToken extends Token {
    private final String literal;

    public StrToken(int line, String str) {
        super(line);
        literal = str;
    }

    @Override
    public boolean isString() {
        return true;
    }

    @Override
    public String getText() {
        return this.literal;
    }
}
