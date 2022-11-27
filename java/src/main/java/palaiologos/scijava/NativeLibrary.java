/*
    scijava
    Copyright (C) 2022 Kamila Szewczyk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

package palaiologos.scijava;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;

class NativeLibrary {
    private static String osName() {
        String os = System.getProperty("os.name").toLowerCase().replace(' ', '_');
        if (os.startsWith("win")){
            return "win";
        } else if (os.startsWith("mac")) {
            return "darwin";
        } else {
            return os;
        }
    }

    private static String osArch() {
        return System.getProperty("os.arch");
    }

    private static String libExtension() {
        if (osName().contains("os_x") || osName().contains("darwin")) {
            return "dylib";
        } else if (osName().contains("win")) {
            return "dll";
        } else {
            return "so";
        }
    }

    private static String resourceName() {
        return "/native/" + osName() + "-" + osArch() + "-scijava." + libExtension();
    }

    private static File temporaryDir;

    private NativeLibrary() {
    }

    private static void load(String path) throws IOException {
        String[] parts = path.split("/");
        String filename = (parts.length > 1) ? parts[parts.length - 1] : null;

        if (temporaryDir == null) {
            temporaryDir = createTempDirectory();
            temporaryDir.deleteOnExit();
        }

        File temp = new File(temporaryDir, filename);

        try (InputStream is = NativeLibrary.class.getResourceAsStream(path)) {
            Files.copy(is, temp.toPath(), StandardCopyOption.REPLACE_EXISTING);
        } catch (IOException e) {
            temp.delete();
            throw e;
        } catch (NullPointerException e) {
            temp.delete();
            throw new FileNotFoundException("File " + path + " was not found inside JAR.");
        }

        try {
            System.load(temp.getAbsolutePath());
        } finally {
            temp.deleteOnExit();
        }
    }

    private static File createTempDirectory() throws IOException {
        String tempDir = System.getProperty("java.io.tmpdir");
        File generatedDir = new File(tempDir, "scijava" + System.nanoTime());

        if (!generatedDir.mkdir())
            throw new IOException("Failed to create temp directory " + generatedDir.getName());

        return generatedDir;
    }

    static {
        try {
            load(resourceName());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
