import React from "react";
import { FaArrowRight } from "react-icons/fa"
import { Link } from "react-router-dom"
import HighlightText from "../components/core/HomePage/HighlightText"
import Button from "../components/core/HomePage/Button";
import CodeBlocks from "../components/core/HomePage/CodeBlocks"
import Footer from "../components/common/Footer"

const Home=()=>{
    return(
        <div>
                <div className="relative mx-auto flex w-11/12 max-w-maxContent flex-col items-center justify-between gap-8 text-white">
                {/* //  {section 1}
            // Heading */}
            <div className=" mt-10 text-center text-4xl font-semibold">
                Borrow USDB instatly using
            <HighlightText text={"BTC "} />
            as Collateral
            </div>
            {/* subheading */}
            <div className="-mt-3 w-[90%] text-center text-lg font-bold text-richblack-300">
            The first decentralised stablecoin, native to the Bitcoin L1.
            </div>

            {/* CTA Buttons */}
            <div className="mt-6 flex flex-row gap-7">
                <Button active={true} linkto={"/signup"}>
                    Learn More
                </Button>
                <Button active={false} linkto={"/login"}>
                    Book a Demo
                </Button>
            </div>

            {/* Code Section 1  */}
            <div>
            <CodeBlocks
                position={"lg:flex-row"}
                heading={
                <div className="text-4xl font-semibold">
                    USDB: Decentralized  
                    <HighlightText text={"BTC-pegged"} /> stablecoin on Runes.
                </div>
                }
                subheading={
                "The USDB token is the first BTC-L1-native decentralised stablecoin, which you can borrow using your BTC as collateral."
                }
                ctabtn1={{
                btnText: "Try it Yourself",
                link: "/signup",
                active: true,
                }}
                ctabtn2={{
                btnText: "Learn More",
                link: "/signup",
                active: false,
                }}
                codeColor={"text-yellow-25"}
                codeblock={`fn main() {\n et mut usdb = Rune::new("USDB", "₿USD",21_000_000, "Satoshi");\nprintln!("Rune: {} | Symbol: {} | Supply: {}", usdb.name, usdb.symbol, usdb.supply);\nusdb.mint(1_000);\n usdb.mint(1_000);\nprintln!("New supply: {}", usdb.supply);\n \n}\n`}
                backgroundGradient={<div className="codeblock1 absolute"></div>}
            />
            </div>

             {/* Code Section 2 */}
        <div>
          <CodeBlocks
            position={"lg:flex-row-reverse"}
            heading={
              <div className="w-[100%] text-4xl font-semibold lg:w-[50%]">
                Powered by
                <HighlightText text={"Runes, REE, and Internet Computer"} />
              </div>
            }
            subheading={
              "USDB is a Bitcoin-backed stablecoin built on Runes, governed by REE, and powered by Internet Computer for scalable, secure, and trustless financial infrastructure."
            }
            ctabtn1={{
              btnText: "Get Started Now",
              link: "/signup",
              active: true,
            }}
            ctabtn2={{
              btnText: "Explore Repos",
              link: "/signup",
              active: false,
            }}
            codeColor={"text-white"}
            codeblock={`struct Storage { num: u32 }\nimpl Storage {\nfn new() -> Self { Storage { num: 0 } \nfn set(&mut self, n: u32) { self.num = n; }\nfn get(&self) -> u32 { self.num }\n}\nfn main() {\nlet mut s = Storage::new();\ns.set(100);\nlet val = s.get();\nprintln!("{}", val);\n}
`}
            backgroundGradient={<div className="codeblock2 absolute"></div>}
          />
        </div>





         


        

         </div>
         {/* Footer */}
          <Footer />
        </div>



    )
    

}


export default Home;