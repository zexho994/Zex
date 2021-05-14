import javax.swing.*;
import java.io.*;

/**
 * @author Zexho
 * @date 2021/5/14 5:30 下午
 */
public class CodeDialog extends Reader {
    private String buffer = null;
    private int pos = 0;

    @Override
    public int read(char[] cBuf, int off, int len) throws IOException {
        if (buffer == null) {
            String in = showDialog();
            if (in == null) {
                return -1;
            } else {
                print(in);
                buffer = in + "\n";
                pos = 0;
            }
        }

        int size = 0;
        int length = buffer.length();
        while (pos < length && size < len) {
            cBuf[off + size++] = buffer.charAt(pos++);
        }
        if (pos == length) {
            buffer = null;
        }
        return size;
    }

    protected void print(String s) {
        System.out.println(s);
    }

    @Override
    public void close() throws IOException {
    }

    protected String showDialog() {
        JTextArea area = new JTextArea(20, 40);
        JScrollPane pane = new JScrollPane(area);
        int result = JOptionPane.showConfirmDialog(
                null, pane, "Input",
                JOptionPane.OK_CANCEL_OPTION,
                JOptionPane.PLAIN_MESSAGE,
                null);
        if (result != JOptionPane.OK_OPTION) {
            return null;
        }
        return area.getText();
    }

    public static Reader file() throws FileNotFoundException {
        JFileChooser chooser = new JFileChooser();
        if (chooser.showOpenDialog(null) == JFileChooser.APPROVE_OPTION) {
            return new BufferedReader(new FileReader(chooser.getSelectedFile()));
        } else {
            throw new FileNotFoundException("no file specified");
        }
    }
}
