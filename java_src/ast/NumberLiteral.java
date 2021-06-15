package ast;

import token.Token;

/**
 * @author Zexho
 * @date 2021/5/17 12:32 下午
 */
public class NumberLiteral extends ASTLeaf {

    public NumberLiteral(Token t) {
        super(t);
    }

    public int value() {
        return token.getNumber();
    }
}
