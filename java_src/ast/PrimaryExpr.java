package ast;

import java.util.List;

/**
 * @author Zexho
 * @date 2021/5/28 8:35 上午
 */
public class PrimaryExpr extends ASTList {
    public PrimaryExpr(List<ASTree> c) {
        super(c);
    }

    public static ASTree create(List<ASTree> c) {
        return c.size() == 1 ? c.get(0) : new PrimaryExpr(c);
    }
}