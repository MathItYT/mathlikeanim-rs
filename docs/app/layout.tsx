import type { Metadata } from "next";
import { Inter }from "next/font/google";
import "./globals.css";
import "highlight.js/styles/github-dark.css";
import Link from "next/link";
import Image from "next/image";
import { ThemeProvider } from "@/components/theme-provider";
import { ModeToggle } from "@/components/ui/mode-toggle";
import { Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle, SheetTrigger } from "@/components/ui/sheet";
import { Menu } from "lucide-react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faGithub } from "@fortawesome/free-brands-svg-icons";
import { Toaster } from "@/components/ui/toaster";


const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "MathLikeAnim-rs",
  description: "A Rust library for interactive math animations",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <link
            rel="preload"
            href="/fonts/Inter-Bold.ttf"
            as="font"
            type="font/ttf"
            crossOrigin="anonymous"
          />
          <script src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/3.2.2/es5/tex-svg-full.js" crossOrigin="anonymous" async id="mathjax-script"></script>
          <div className="fixed top-0 left-0 w-16 h-full">
            <ModeToggle />
            <Sheet>
              <SheetTrigger>
                <Menu className="mt-3 ml-5"/>
              </SheetTrigger>
              <SheetContent side="left">
                <SheetHeader>
                  <SheetTitle asChild>
                    <Link href="/">
                      MathLikeAnim-rs
                    </Link>
                  </SheetTitle>
                  <SheetDescription>
                    A Rust library for interactive math animations
                  </SheetDescription>
                  <div className="flex flex-row">
                    <Link href="https://github.com/MathItYT/mathlikeanim-rs">
                      <FontAwesomeIcon icon={faGithub} className="h-[4vh]" />
                    </Link>
                  </div>
                  <div className="flex flex-col pl-[1vh] pt-[1vh]">
                    <Link href={`/examples`}>
                      Examples
                    </Link>
                  </div>
                </SheetHeader>
              </SheetContent>
            </Sheet>
          </div>
          <div className="flex flex-col justify-center items-center pt-[1vh]">
            <Link href="/">
              <Image
                className="w-[10vw] rounded-lg"
                src="/banner.png"
                alt="MathLikeAnim-rs Logo"
                title="MathLikeAnim-rs Logo"
                width={1280}
                height={640}
                priority
              />
            </Link>
          </div>
          <div className="flex flex-col pt-[5vh]">
            {children}
          </div>
          <Toaster />
        </ThemeProvider>
      </body>
    </html>
  );
}
