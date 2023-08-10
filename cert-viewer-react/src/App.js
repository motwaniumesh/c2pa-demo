import { createC2pa } from "c2pa";
import React, { useEffect, useState } from "react";
import ReactMarkdown from "react-markdown";
import UploadFileIcon from "@mui/icons-material/UploadFile";
//Pulling components in individually for performance reasons. May want to verify the performance increases.
import Typography from "@mui/material/Typography";
import Card from "@mui/material/Card";
import Box from "@mui/material/Box";
import CardActionArea from "@mui/material/CardActionArea";
import IconButton from "@mui/material/IconButton";
import CardContent from "@mui/material/CardContent";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import DisplayImage from "./components/DisplayImage";

//TODOs:
//- Create the dropzone with logic
//    - NOTE: should be able to cancel out of dropped image to a default "imageless" state of the application
//- Card
//    - size/proprtions should be consistent across viewports -- mobile, desktop, tablet, etc etc
//    - Try some styling to make the information Icon more prominent (maybe glowing??)
//    - The information icon and dialog should do be responsive if there's no provenance data on a dropzoned image.....
//- Add instructions/blog information to the Markdown section
//- Refactor so that the popper is not within the CardActionArea component (clicking the information dialog has an effect on the image!)
//- Build a provenance history component in the popper
//    - Research how to add company icons similar to the model
//    - Make it CLEAR that the provenance is a "history" (more clear than the template). Maybe adding the word history to the title?
//- Resizable border between the content (If too hard don't do....)
//- The icons are blurry -- this might be a font issue
function App() {
  const [fullManifest, setManifest] = useState("");

  const test_image_url =
    //TODO: Add an eye catching default image
    "signed_media.jpg";

  //Some other images worth using for testing:
  //  Signed with multiple manifests:
  //        "https://raw.githubusercontent.com/contentauth/c2pa-js/main/tools/testing/fixtures/images/CAICAI.jpg";
  //  Unsigned
  //        "https://bafybeiefhf7fydgjav3ltwko27kqdw7aztpfyq3yrrmlpysrjzmnlavyie.ipfs.nftstorage.link/";

  useEffect(() => {
    const create = async () => {
      try {
        const instance = await createC2pa({
          wasmSrc:
            "https://cdn.jsdelivr.net/npm/c2pa@0.15.0/dist/assets/wasm/toolkit_bg.wasm",
          workerSrc:
            "https://cdn.jsdelivr.net/npm/c2pa@0.15.0/dist/c2pa.worker.min.js",
        });

        const { manifestStore } = await instance.read(test_image_url);
        const newStore = manifestStore.activeManifest.signatureInfo ?? "failed";
        const date = new Date(newStore.time);

        setManifest({
          ...newStore,
          time: date.toLocaleDateString("en-US", {
            year: "numeric",
            month: "long",
            day: "numeric",
            // hour: "numeric",
            // minute: "numeric",
            // second: "numeric",
            // timeZoneName: "short",
          }),
        });
      } catch (e) {
        console.log(`failed to create ${e}`);
      } finally {
      }
    };

    create();
  }, []);

  const mdTemp = `*Drop or embed the blog post here* \n\n Remember to add Slalom Logo`;

  return (
    <Box
      sx={{
        paddingLeft: 3,
        display: "flex",
        alignItems: "center",
        height: "100vh",
        backgroundColor: "rgba(246, 246, 246)",
      }}
    >
      <Grid
        item
        width={"45vw"}
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <Card sx={{ width: "35vw" }}>
          <CardContent>
            <Typography variant="h6">
              <b>Show Media Provenance on Your Site</b>
            </Typography>
            <Typography variant="caption" color="gray">
              <i>Click the info icon to view the content's secure history</i>{" "}
            </Typography>
          </CardContent>

          <CardActionArea
            sx={{
              display: "flex",
              justifyContent: "center",
              overflow: "hidden",
              borderRadius: 0,
            }}
          >
            <DisplayImage url={test_image_url} manifest={fullManifest} />
          </CardActionArea>

          <CardContent sx={{ padding: 3 }}>
            <Grid item padding={1}>
              <Typography variant="subtitle1" textAlign={"center"}>
                Drag and drop an image below to load it into this site and read
                it's C2PA provenance data.
              </Typography>
            </Grid>
            <Grid item padding={1}>
              <Paper elevation={1} sx={{ borderRadius: 2 }}>
                <IconButton
                  sx={{ height: "3em", width: "100%", borderRadius: 0 }}
                >
                  <UploadFileIcon sx={{ fontSize: 60 }} />
                </IconButton>
              </Paper>
            </Grid>
          </CardContent>
        </Card>
      </Grid>

      {/* Markdown component */}
      <Grid
        item
        width={"68vw"}
        height={"100vh"}
        sx={{ borderLeft: ".5rem solid lightgray", backgroundColor: "white" }}
      >
        <Box padding={2}>
          <ReactMarkdown>{mdTemp}</ReactMarkdown>
        </Box>
      </Grid>
    </Box>
  );
}

export default App;
